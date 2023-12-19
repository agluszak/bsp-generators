load("@io_bazel_rules_scala//scala:scala.bzl", "scala_test", "scala_binary")

scala_test(
    name = "tests",
    srcs = glob(["src/test/scala/**/*.scala"]),
    deps = [
        "//bsp4s",
        "//bsp4j",
        "//testkit",
        "@maven//:com_googlecode_java_diff_utils_diffutils",
        "@maven//:org_scala_lang_modules_scala_java8_compat_2_13",
        "@maven//:org_scala_sbt_ipcsocket_ipcsocket",
        "@maven//:org_scalatest_scalatest_2_13",
        "@maven//:org_scalatestplus_scalacheck_1_16_2_13",
        "@maven//:org_scalacheck_scalacheck_2_13",
        "@maven//:com_github_plokhotnyuk_jsoniter_scala_jsoniter_scala_core_2_13",
        "@maven//:me_vican_jorge_jsonrpc4s_2_13",
        "@maven//:com_google_code_gson_gson",
        "@maven//:com_google_guava_guava",
        "@maven//:org_eclipse_lsp4j_org_eclipse_lsp4j_jsonrpc",
        "@maven//:io_monix_monix_2_13",
        "@maven//:io_monix_monix_execution_2_13",
        "@maven//:io_monix_monix_eval_2_13",
        "@maven//:com_outr_scribe_2_13",
    ],
    visibility = ["//visibility:public"],
)

load("@rules_kotlin//kotlin:kotlin.bzl", "kt_jvm_binary")


kt_jvm_binary(
    name = "kotlin_json_roundtrip",
    main_class = "org.jetbrains.bsp.tests.KotlinJsonRoundtripKt",
    visibility = ["//visibility:public"],
    srcs = glob(["src/main/kotlin/**/*.kt"]),
    deps = [
        "//bsp4kt",
        "@//:kotlin_serialization",
    ],
)

java_binary(
    name = "java_json_roundtrip",
    main_class = "org.jetbrains.bsp.tests.JavaJsonRoundtrip",
    visibility = ["//visibility:public"],
    srcs = glob(["src/main/java/**/*.java"]),
    deps = [
        "//bsp4j",
        "@maven//:com_google_code_gson_gson",
        "@maven//:org_eclipse_lsp4j_org_eclipse_lsp4j_jsonrpc",
    ],
)

scala_binary(
    name = "scala_json_roundtrip",
    main_class = "org.jetbrains.bsp.tests.ScalaJsonRoundtrip",
    visibility = ["//visibility:public"],
    srcs = glob(["src/main/scala/**/*.scala"]),
    deps = [
        "//bsp4s",
        "@maven//:com_github_plokhotnyuk_jsoniter_scala_jsoniter_scala_core_2_13",
        "@maven//:com_github_plokhotnyuk_jsoniter_scala_jsoniter_scala_macros_2_13",
    ],
)

load("//rules/kotlin:junit5.bzl", "kt_test")

kt_test(
    name = "serialization_test",
    src = "src/test/kotlin/SerializationTest.kt",
    classname = "SerializationTest",
    data = [
        "//tests:java_json_roundtrip",
        "//tests:kotlin_json_roundtrip",
        "//tests:scala_json_roundtrip",
    ],
    deps = [
        "//codegen",
        "@//:kotlin_serialization",
        "@maven//:com_github_agluszak_jsonrpc4kt",
        "@maven//:software_amazon_smithy_smithy_codegen_core",
        "@maven//:software_amazon_smithy_smithy_model",
        "@maven//:com_fasterxml_jackson_core_jackson_annotations",
        "@maven//:com_fasterxml_jackson_core_jackson_core",
        "@maven//:com_fasterxml_jackson_core_jackson_databind",
    ],
)
