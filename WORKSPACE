load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

bazel_skylib_version = "1.4.1"
bazel_skylib_sha = "b8a1527901774180afc798aeb28c4634bdccf19c4d98e7bdd1ce79d1fe9aaad7"

http_archive(
    name = "bazel_skylib",
    sha256 = bazel_skylib_sha,
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/%s/bazel-skylib-%s.tar.gz" % (bazel_skylib_version, bazel_skylib_version),
        "https://github.com/bazelbuild/bazel-skylib/releases/download/%s/bazel-skylib-%s.tar.gz" % (bazel_skylib_version, bazel_skylib_version),
    ],
)

rules_scala_version = "6.1.0"
rules_scala_sha = "cc590e644b2d5c6a87344af5e2c683017fdc85516d9d64b37f15d33badf2e84c"

http_archive(
    name = "io_bazel_rules_scala",
    sha256 = rules_scala_sha,
    strip_prefix = "rules_scala-%s" % rules_scala_version,
    url = "https://github.com/bazelbuild/rules_scala/releases/download/v%s/rules_scala-v%s.tar.gz" % (rules_scala_version, rules_scala_version),
)


load("@io_bazel_rules_scala//:scala_config.bzl", "scala_config")
# Stores Scala version and other configuration
# 2.12 is a default version, other versions can be use by passing them explicitly:
# scala_config(scala_version = "2.11.12")
# Scala 3 requires extras...
#   3.2 should be supported on master. Please note that Scala artifacts for version (3.2.2) are not defined in
#   Rules Scala, they need to be provided by your WORKSPACE. You can use external loader like
#   https://github.com/bazelbuild/rules_jvm_external
scala_config(scala_version = "2.13.6")

load("@io_bazel_rules_scala//scala:scala.bzl", "rules_scala_setup", "rules_scala_toolchain_deps_repositories")

# loads other rules Rules Scala depends on
rules_scala_setup()

# Loads Maven deps like Scala compiler and standard libs. On production projects you should consider
# defining a custom deps toolchains to use your project libs instead
rules_scala_toolchain_deps_repositories(fetch_sources = True)

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")
rules_proto_dependencies()
rules_proto_toolchains()

load("@io_bazel_rules_scala//scala:toolchains.bzl", "scala_register_toolchains")
scala_register_toolchains()

load("@io_bazel_rules_scala//testing:scalatest.bzl", "scalatest_repositories", "scalatest_toolchain")
scalatest_repositories()
scalatest_toolchain()

rules_kotlin_version = "1.8"
rules_kotlin_sha = "01293740a16e474669aba5b5a1fe3d368de5832442f164e4fbfc566815a8bc3a"

http_archive(
    name = "io_bazel_rules_kotlin",
    sha256 = rules_kotlin_sha,
    urls = ["https://github.com/bazelbuild/rules_kotlin/releases/download/v%s/rules_kotlin_release.tgz" % rules_kotlin_version],
)

load("@io_bazel_rules_kotlin//kotlin:repositories.bzl", "kotlin_repositories")

kotlin_repositories()  # if you want the default. Otherwise see custom kotlinc distribution below

load("@io_bazel_rules_kotlin//kotlin:core.bzl", "kt_register_toolchains")

kt_register_toolchains()  # to use the default toolchain, otherwise see toolchains below

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")

new_git_repository(
    name = "bsp",
    build_file = "@//:BUILD.bsp.bazel",
    commit = "5af516738f17dbb03bec931a22027c99ffd225ff",
    remote = "git@github.com:build-server-protocol/build-server-protocol.git",
)

RULES_JVM_EXTERNAL_TAG = "5.3"

RULES_JVM_EXTERNAL_SHA = "d31e369b854322ca5098ea12c69d7175ded971435e55c18dd9dd5f29cc5249ac"

http_archive(
    name = "rules_jvm_external",
    sha256 = RULES_JVM_EXTERNAL_SHA,
    strip_prefix = "rules_jvm_external-{}".format(RULES_JVM_EXTERNAL_TAG),
    url = "https://github.com/bazelbuild/rules_jvm_external/releases/download/%s/rules_jvm_external-%s.tar.gz" % (RULES_JVM_EXTERNAL_TAG, RULES_JVM_EXTERNAL_TAG),
)

load("@rules_jvm_external//:repositories.bzl", "rules_jvm_external_deps")

rules_jvm_external_deps()

load("@rules_jvm_external//:setup.bzl", "rules_jvm_external_setup")

rules_jvm_external_setup()

load("@rules_jvm_external//:defs.bzl", "maven_install")

maven_install(
    artifacts = [
        "software.amazon.smithy:smithy-model:1.34.0",
        "software.amazon.smithy:smithy-codegen-core:1.34.0",
        "software.amazon.smithy:smithy-utils:1.34.0",

        # tests
        "org.junit.jupiter:junit-jupiter:5.10.0",
        "org.junit.jupiter:junit-jupiter-api:5.10.0",
        "org.junit.jupiter:junit-jupiter-engine:5.10.0",
        "org.junit.jupiter:junit-jupiter-params:5.10.0",
        "org.junit.platform:junit-platform-console:1.10.0",

        # bsp4kt
        "org.jetbrains.kotlinx:kotlinx-serialization-json:1.5.1",
        "com.github.agluszak:jsonrpc4kt:f4d0c972f8",

        # scala
        "org.scala-lang.modules:scala-collection-compat_2.13:2.11.0",
        "com.lihaoyi:os-lib_2.13:0.9.1",
        "com.lihaoyi:geny_2.13:1.0.0",
        "com.monovore:decline_2.13:2.4.1",
        "org.typelevel:cats-core_2.13:2.9.0",
        "org.typelevel:cats-kernel_2.13:2.9.0",

        # scala runtime libs
        "com.github.plokhotnyuk.jsoniter-scala:jsoniter-scala-core_2.13:2.23.2",
        "com.github.plokhotnyuk.jsoniter-scala:jsoniter-scala-macros_2.13:2.23.2",
        "me.vican.jorge:jsonrpc4s_2.13:0.1.0",

        # lsp4j 21.1 causes "b = new (this);" bug (missing ToStringBuilder)
        "org.eclipse.lsp4j:org.eclipse.lsp4j:0.20.1",
        "org.eclipse.lsp4j:org.eclipse.lsp4j.generator:0.20.1",
        "org.eclipse.lsp4j:org.eclipse.lsp4j.jsonrpc:0.20.1",

        # lsp4j deps versions must be aligned with lsp4j version
        "org.eclipse.xtend:org.eclipse.xtend.core:2.28.0",
        "org.eclipse.xtend:org.eclipse.xtend.lib:2.28.0",
        "org.eclipse.xtext:org.eclipse.xtext.xbase.lib:2.28.0",
        "com.google.guava:guava:30.1.1-jre",
        "com.google.inject:guice:7.0.0",

    ],
    fetch_sources = True,
    repositories = [
        "https://maven.google.com",
        "https://repo.maven.apache.org/maven2",
        "https://repo1.maven.org/maven2",
        "https://jitpack.io",
    ],
)

http_archive(
    name = "rules_rust",
    sha256 = "9d04e658878d23f4b00163a72da3db03ddb451273eb347df7d7c50838d698f49",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.26.0/rules_rust-v0.26.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:bsp4rs/Cargo.lock",
    lockfile = "//:bsp4rs/cargo-bazel-lock.json",
    manifests = [
        "//:bsp4rs/Cargo.toml",
    ],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

http_archive(
    name = "rules_multirun",
    sha256 = "9cd384e42b2da00104f0e18f25e66285aa21f64b573c667638a7a213206885ab",
    strip_prefix = "rules_multirun-0.6.1",
    url = "https://github.com/keith/rules_multirun/archive/refs/tags/0.6.1.tar.gz",
)
