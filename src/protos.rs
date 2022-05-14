pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("bache_descriptor");

pub mod build {
    pub mod bazel {
        pub mod semver {
            tonic::include_proto!("build.bazel.semver");
        }

        pub mod remote {
            pub mod asset {
                pub mod v1 {
                    tonic::include_proto!("build.bazel.remote.asset.v1");
                }
            }

            pub mod execution {
                pub mod v2 {
                    tonic::include_proto!("build.bazel.remote.execution.v2");
                }
            }

            pub mod logstream {
                pub mod v1 {
                    tonic::include_proto!("build.bazel.remote.logstream.v1");
                }
            }
        }
    }
}

pub mod google {
    pub mod api {
        tonic::include_proto!("google.api");
    }

    pub mod bytestream {
        tonic::include_proto!("google.bytestream");
    }

    pub mod longrunning {
        tonic::include_proto!("google.longrunning");
    }

    pub mod protobuf {
        tonic::include_proto!("google.protobuf");
    }

    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
}
