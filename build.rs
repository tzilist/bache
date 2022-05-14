use std::{env, path::PathBuf};

use eyre::WrapErr;
use glob::glob;

fn main() -> eyre::Result<()> {
    let file_descriptor_path =
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("bache_descriptor.bin");

    let protos = glob("protos/**/*.proto")
        .wrap_err("Failed to read glob pattern")?
        .into_iter()
        .map(|file| {
            // rerun the build if any of the protos files change
            let file = file.unwrap();
            println!("cargo:rerun-if-changed={:?}", file.display());

            file
        })
        .collect::<Vec<_>>();

    tonic_build::configure()
        .file_descriptor_set_path(file_descriptor_path)
        .build_client(true)
        .build_server(true)
        .compile(&protos, &["protos"])?;

    Ok(())
}
