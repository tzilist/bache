workspace(name = "bache")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# PROTOBUF

http_archive(
    name = "com_google_protobuf",
    sha256 = "8b28fdd45bab62d15db232ec404248901842e5340299a57765e48abe8a80d930",
    strip_prefix = "protobuf-3.20.1",
    urls = [
        "https://mirror.bazel.build/github.com/protocolbuffers/protobuf/archive/v3.20.1.tar.gz",
        "https://github.com/protocolbuffers/protobuf/archive/v3.20.1.tar.gz",
    ],
)

load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")

protobuf_deps()

# RUST

http_archive(
    name = "rules_rust",
    sha256 = "edb87c0d2ba70823fe3df7862676d695599314a4634b9758bd55f0e8f19c2751",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_rust/releases/download/0.4.0/rules_rust-v0.4.0.tar.gz",
        "https://github.com/bazelbuild/rules_rust/releases/download/0.4.0/rules_rust-v0.4.0.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    include_rustc_srcs = True,
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies(
    bootstrap = True,
    rust_version = "1.60.0",
)

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")

crates_repository(
    name = "bache_crates",
    annotations = {
        # "jemalloc-sys": [crate.annotation(
        #     gen_build_script = False,
        #     deps = ["@jemalloc"],
        # )],
        "opentelemetry-otlp": [crate.annotation(
            build_script_env = {
                "PROTOC": "$(execpath @com_google_protobuf//:protoc)",
            },
            build_script_tools = ["@com_google_protobuf//:protoc"],
        )],
        "prost-build": [crate.annotation(
            build_script_env = {
                "PROTOC": "$(execpath @com_google_protobuf//:protoc)",
            },
            build_script_tools = ["@com_google_protobuf//:protoc"],
        )],
        "tonic-health": [crate.annotation(
            build_script_env = {
                "PROTOC": "$(execpath @com_google_protobuf//:protoc)",
            },
            build_script_tools = ["@com_google_protobuf//:protoc"],
        )],
        "tonic-reflection": [crate.annotation(
            build_script_env = {
                "PROTOC": "$(execpath @com_google_protobuf//:protoc)",
            },
            build_script_tools = ["@com_google_protobuf//:protoc"],
        )],
    },
    generator = "@cargo_bazel_bootstrap//:cargo-bazel",
    lockfile = "//:Cargo.lock",
    lockfile_kind = "cargo",
    manifests = ["//:Cargo.toml"],
)

load("@bache_crates//:defs.bzl", "crate_repositories")

crate_repositories()
