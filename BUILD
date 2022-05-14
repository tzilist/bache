load("@bache_crates//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")

cargo_build_script(
    name = "build_script",
    srcs = ["build.rs"],
    build_script_env = {
        "PROTOC": "$(execpath @com_google_protobuf//:protoc)",
        "PROTOC_INCLUDE": "${pwd}/external/com_google_protobuf/src",
    },
    data = [
        "@com_google_protobuf//:well_known_protos",
    ] + glob(["protos/**/*.proto"]),
    edition = "2021",
    tools = [
        "@com_google_protobuf//:protoc",
    ],
    deps = all_crate_deps(build = True),
)

rust_library(
    name = "bache_lib",
    srcs = glob(["src/**/*.rs"]),
    crate_name = "bache",
    edition = "2021",
    proc_macro_deps = all_crate_deps(proc_macro = True),
    deps = all_crate_deps(normal = True) + [":build_script"],
)

rust_binary(
    name = "bin",
    srcs = ["src/main.rs"],
    aliases = aliases(),
    edition = "2021",
    proc_macro_deps = all_crate_deps(proc_macro = True),
    deps = all_crate_deps(normal = True) + [":bache_lib"],
)
