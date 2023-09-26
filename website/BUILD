load("@npm//:defs.bzl", "npm_link_all_packages")
load("@npm//website:@docusaurus/core/package_json.bzl", docusaurus_bin = "bin")

load("@aspect_rules_js//js:defs.bzl", "js_run_binary", "js_binary")

npm_link_all_packages(name = "node_modules")

filegroup(
    name = "docs",
    srcs = glob(["generated/**/*", "src/**/*", "static/**/*", "website/**/*"]),
    visibility = ["//visibility:public"],
)

docusaurus_bin.docusaurus_binary(
    name = "docusaurus",
    args = ["start", "--config", "docusaurus.config.js"],
    chdir = package_name(),
    data =
    glob(["docs/**/*", "src/**/*", "static/**/*"]) +
    [
        "sidebars.json",
        "docusaurus.config.js",
        "package.json",
        ":node_modules/@docusaurus/preset-classic",
        ":node_modules/@docusaurus/plugin-client-redirects",
        ":node_modules/@easyops-cn/docusaurus-search-local",
        ":node_modules/react",
        ":node_modules/@mdx-js/react",
    ],
    visibility = ["//visibility:public"],
)
