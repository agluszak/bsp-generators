load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "bsp4rs",
    srcs = glob(["src/**/*.rs"]),
    crate_name = "bsp4rs",
    proc_macro_deps = [
        "@crate_index//:serde-enum-str",
        "@crate_index//:serde_repr",
    ],
    deps = [
        "@crate_index//:lsp-types",
        "@crate_index//:serde",
        "@crate_index//:serde_json",
    ],
)
